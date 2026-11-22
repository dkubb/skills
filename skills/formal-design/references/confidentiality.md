# Confidentiality and sealed interfaces

Use these questions when a public interface handles private state or makes
a claim about what callers may learn.

1. **Who observes what?** Name the caller, the protected fact, and the
   observations the contract permits. Include result values, errors and
   any observable changes that affect later calls.
2. **Which states must appear identical?** Choose two states with equal
   public inputs and different protected values. Specify the permitted
   relation between the results and between the successor states.
   For example, a lookup can disclose existence if its two failure cases
   are distinguishable to someone lacking read access.
3. **What can a client do next?** Cover repeated calls, including a second
   input chosen after the first result. State whether timing and resource
   use are observable and where any assumptions about them are enforced.
4. **What does the exported type reveal?** Sketch the smallest public
   client and enumerate its projections, eliminators, derived instances
   and conversions. Give each observation a contract. Prefer storing only
   the capability the client needs when that suffices.
5. **Which disclosures are deliberate?** Record the fact, recipient,
   authorized operation and release condition. Specify who controls that
   condition and how untrusted inputs are prevented from authorizing
   their own release or becoming trusted evidence.

Use the [information-flow review questions](../../find-and-prove/references/information-flow.md)
to check the implemented interface against these observations.
