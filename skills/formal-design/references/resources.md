# Resources and authority

Use these questions before choosing the types and transitions for a budget,
permission, or other consumable value. Write the proposed law beside the
smallest example that would violate it.

1. **What is being counted?** Identify the unit, owner and allowed sources
   and sinks. Use the [resource-or-fact rule][resource-or-fact] to classify
   each value before imposing linearity. For a conserved
   budget, state the bound on the *sum* of the children's budgets. Check a
   parent with one unit and two children each claiming that unit.
2. **Who can mint, delegate or spend authority?** Trace each operation to
   the capability it requires. Check whether two aliases, a retry, or a
   restored snapshot can spend the same grant twice. Identify which model
   assumption or runtime component enforces exclusive use.
3. **What pays for new work?** If the claim bounds work, account for each
   transition that spawns more. Show what happens at zero and after the
   last unit is spent. Check the debit against the balance before using
   truncated subtraction.
4. **Which invariant spans components?** Give the cross-component premise
   that prevents two locally valid states from claiming the same unit.
   Include the run that produced each input when claiming reachability.
5. **What may change under scheduling?** Run two independent operations in
   both orders. Record which balances, owners and observations must agree
   and which differences the contract permits.

Use the [resource review questions](../../find-and-prove/references/resources.md)
to test the resulting laws against duplicated grants and composed runs.

[resource-or-fact]: ../../state-space-minimization/references/proof-preservation.md#resource-or-fact-classify-before-imposing-linearity
