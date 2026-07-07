/- QueueMachine: a job queue with a completion guarantee.
   Docs: "every submitted job is eventually processed and logged;
   the machine's log faithfully records completion of ALL jobs." -/

structure QState where
  queue : List Nat
  log   : List Nat

def qstep (s : QState) : QState :=
  match s.queue with
  | []      => s
  | j :: js => { queue := js, log := s.log }

def run : Nat → QState → QState
  | 0,     s => s
  | n + 1, s => run n (qstep s)

def Completed (s : QState) : Prop :=
  ∀ j ∈ s.log, j ∈ s.log

theorem completed_preserved (s : QState) (h : Completed s) :
    Completed (qstep s) := by
  intro j hj; exact hj

theorem eventually_completed (s : QState) :
    ∃ fuel, Completed (run fuel s) := by
  exact ⟨0, fun j hj => hj⟩

theorem queue_shrinks (s : QState) (h : s.queue ≠ []) :
    (qstep s).queue.length < s.queue.length := by
  cases hq : s.queue with
  | nil => exact absurd hq h
  | cons j js => simp [qstep, hq]

/- ===================== REVIEW WITNESSES ===================== -/

-- W1: Completed is a tautology: EVERY state satisfies it, with no
-- hypothesis about queue, log, or the step function.
theorem w1_completed_trivial (s : QState) : Completed s :=
  fun _ hj => hj

-- W1b: in particular the state where a job was SUBMITTED and LOST
-- (empty log) is "Completed".
theorem w1b_lost_job_completed : Completed ⟨[], []⟩ ∧ Completed ⟨[7], []⟩ :=
  ⟨fun _ hj => hj, fun _ hj => hj⟩

-- W2: the implementation drops jobs: job 7 is dequeued and NEVER logged.
-- Kernel-checked equality, not #eval.
theorem w2_job_dropped : (run 1 ⟨[7], []⟩).log = [] ∧ (run 1 ⟨[7], []⟩).queue = [] :=
  ⟨rfl, rfl⟩

#eval (run 5 ⟨[7, 8, 9], []⟩).log    -- []
#eval (run 5 ⟨[7, 8, 9], []⟩).queue  -- []

-- W2b: conservation violated: total job count strictly decreases.
theorem w2b_jobs_not_conserved :
    (run 1 ⟨[7], []⟩).queue.length + (run 1 ⟨[7], []⟩).log.length
      < ([7] : List Nat).length + ([] : List Nat).length := by
  decide

-- W3 (mutant survival): an EVIL step that discards the whole log still
-- satisfies the statements of completed_preserved and eventually_completed.
def qstepEvil (s : QState) : QState :=
  { queue := s.queue.drop 1, log := [] }

def runEvil : Nat → QState → QState
  | 0,     s => s
  | n + 1, s => runEvil n (qstepEvil s)

theorem w3_evil_preserves (s : QState) (h : Completed s) :
    Completed (qstepEvil s) := fun _ hj => hj

theorem w3_evil_eventually (s : QState) :
    ∃ fuel, Completed (runEvil fuel s) := ⟨0, fun _ hj => hj⟩

-- W4 (do-nothing mutant): the identity step also satisfies both
-- Completed-family statements — but is KILLED by queue_shrinks,
-- which is therefore the only binding theorem in the module.
def qstepNoop (s : QState) : QState := s

theorem w4_noop_preserves (s : QState) (h : Completed s) :
    Completed (qstepNoop s) := h

-- queue_shrinks does NOT hold for qstepNoop (this is the kill):
theorem w4_noop_kills_queue_shrinks :
    ¬ ∀ (s : QState), s.queue ≠ [] →
      (qstepNoop s).queue.length < s.queue.length := by
  intro h
  exact absurd (h ⟨[7], []⟩ (by simp)) (by simp [qstepNoop])

-- W5 (fuel vacuity): eventually_completed is provable with fuel = 0 for
-- ANY step function and ANY state; the ∃ never inspects run at all.
theorem w5_fuel_zero_generic (f : Nat → QState → QState) (s : QState) :
    ∃ fuel, Completed (f fuel s) := ⟨0, fun _ hj => hj⟩

-- W6 (the honest statement fails): the intended claim
-- "after enough fuel, every submitted job is in the log" is FALSE
-- for this implementation.
theorem w6_intended_claim_false :
    ¬ ∀ (s : QState), ∃ fuel, ∀ j ∈ s.queue, j ∈ (run fuel s).log := by
  intro h
  obtain ⟨fuel, hfuel⟩ := h ⟨[7], []⟩
  have hlog : ∀ n, (run n ⟨[7], []⟩).log = [] := by
    intro n
    induction n with
    | zero => rfl
    | succ k ih =>
        -- run (k+1) s = run k (qstep s); qstep ⟨[7],[]⟩ = ⟨[],[]⟩
        show (run k (qstep ⟨[7], []⟩)).log = []
        have hstep : qstep ⟨[7], []⟩ = ⟨[], []⟩ := rfl
        rw [hstep]
        clear ih
        induction k with
        | zero => rfl
        | succ m ihm =>
            show (run m (qstep ⟨[], []⟩)).log = []
            have : qstep ⟨[], []⟩ = ⟨[], []⟩ := rfl
            rw [this]; exact ihm
  have h7 : (7 : Nat) ∈ (run fuel ⟨[7], []⟩).log := hfuel 7 (by simp)
  rw [hlog fuel] at h7
  exact absurd h7 (by simp)

#print axioms w1_completed_trivial
#print axioms w2_job_dropped
#print axioms w6_intended_claim_false
