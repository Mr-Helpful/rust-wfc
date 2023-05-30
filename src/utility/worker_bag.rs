use std::{
  collections::TryReserveError,
  hash::{Hash, Hasher},
  ops::Deref,
  sync::RwLock,
};

use rayon::join;

/// An error representing either a failure from the task we're performing
/// or from the worker bag itself (the bag'll only fail when it's filled
/// completely and we attempt to add another task).
pub enum WorkerBagError<E> {
  WorkerError(E),
  BagFullError(TryReserveError),
}

impl<E> WorkerBagError<E> {
  pub fn worker(value: E) -> Self {
    Self::WorkerError(value)
  }
}

impl<E> From<TryReserveError> for WorkerBagError<E> {
  fn from(value: TryReserveError) -> Self {
    WorkerBagError::BagFullError(value)
  }
}

type WorkerBagResult<E> = Result<(), WorkerBagError<E>>;

/// A collection of tasks that can be run in parallel<br>
/// This is intended to be used when parallel updates may trigger other updates.
#[derive(Debug)]
pub struct WorkerBag<T> {
  failed: RwLock<bool>,
  tasks: RwLock<Vec<T>>,
}

const JUSTIFICATION: &str = r#"
We only ever `.pop`, `.push` or `clear` with this lock
and we check whether `.push` can reserve beforehand.
Therefore, we'll never panic whilst writing to this lock.
"#;

impl<T: Clone> Clone for WorkerBag<T> {
  fn clone(&self) -> Self {
    Self {
      failed: RwLock::new(*self.failed.read().unwrap()),
      tasks: RwLock::new(self.tasks.read().unwrap().clone()),
    }
  }
}

impl<T> Default for WorkerBag<T> {
  fn default() -> Self {
    Self {
      failed: RwLock::new(false),
      tasks: RwLock::new(Vec::new()),
    }
  }
}

impl<T: PartialEq> PartialEq<WorkerBag<T>> for WorkerBag<T> {
  fn eq(&self, other: &WorkerBag<T>) -> bool {
    (self
      .failed
      .read()
      .ok()
      .zip(other.failed.read().ok())
      .map_or(false, |(failed0, failed1)| failed0.eq(&failed1)))
      && (self
        .tasks
        .read()
        .ok()
        .zip(other.tasks.read().ok())
        .map_or(false, |(tasks0, tasks1)| tasks0.eq(tasks1.deref())))
  }
}
impl<T: PartialEq> Eq for WorkerBag<T> {}

impl<T: Hash> Hash for WorkerBag<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.failed.read().unwrap().hash(state);
    self.tasks.read().unwrap().hash(state)
  }
}

impl<T> WorkerBag<T> {
  /// Attempts to add an iterable of tasks to the pending tasks.
  fn try_add_tasks<E>(&self, tasks: impl IntoIterator<Item = T>) -> WorkerBagResult<E> {
    let mut all_tasks = self.tasks.write().expect(JUSTIFICATION);
    for task in tasks.into_iter() {
      all_tasks.try_reserve(1)?;
      all_tasks.push(task);
    }
    Ok(())
  }

  /// Attempts to run a task and add its results to the stack of pending tasks.
  ///
  /// # Panics
  /// Only when the worker function panics.
  fn try_run_task<R, E>(&self, worker: impl Fn(T) -> Result<R, E>, task: T) -> WorkerBagResult<E>
  where
    R: IntoIterator<Item = T>,
  {
    self.try_add_tasks(worker(task).map_err(WorkerBagError::WorkerError)?)
  }
}

impl<T: Send + Sync> WorkerBag<T> {
  fn try_run_tasks<E, R>(&self, worker: impl Fn(T) -> Result<R, E> + Sync) -> WorkerBagResult<E>
  where
    R: IntoIterator<Item = T>,
    E: Send,
  {
    let last = self.tasks.write().expect(JUSTIFICATION).pop();
    if last.is_none() || *self.failed.read().expect(JUSTIFICATION) {
      return Ok(());
    }

    let task = last.expect("If last is empty, we'll have returned");
    self.try_run_task(&worker, task).map_err(|err| {
      // set the failing tag to ensure all other threads terminate
      let mut failed = self
        .failed
        .write()
        .expect("We won't panic whilst writing true...");
      *failed = true;
      err
    })?;
    let results = join(
      || self.try_run_tasks(&worker),
      || self.try_run_tasks(&worker),
    );
    results.0.and(results.1)
  }

  /// Fully resets the bag's logic, such that it can be reused on new tasks.
  pub fn reset(&self) {
    *self
      .failed
      .write()
      .expect("We'll only set a flag on writing") = false;
    self.tasks.write().expect(JUSTIFICATION).clear();
  }

  /// Runs a worker function on all current tasks in the bag.
  /// Returns an error if the worker errors on any of the tasks.
  /// Otherwise adds all the tasks the worker produces to the bag.
  ///
  /// # Panics
  /// Only when the worker function panics.
  pub fn run<R, E>(&self, worker: impl Fn(T) -> Result<R, E> + Sync) -> WorkerBagResult<E>
  where
    R: IntoIterator<Item = T>,
    E: Send,
  {
    let result = self.try_run_tasks(worker);
    self.reset();
    result
  }

  /// Runs a worker function on a set of given tasks.
  /// Returns an error if the worker errors on any of the tasks.
  /// Otherwise adds all the tasks the worker produces to the bag.
  ///
  /// # Panics
  /// Only when the worker function panics.
  pub fn run_on<R, E>(
    &self,
    tasks: impl IntoIterator<Item = T>,
    worker: impl Fn(T) -> Result<R, E> + Sync,
  ) -> WorkerBagResult<E>
  where
    R: IntoIterator<Item = T>,
    E: Send,
  {
    self.reset();
    self.try_add_tasks(tasks)?;
    self.run(worker)
  }
}
