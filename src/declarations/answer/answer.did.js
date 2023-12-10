export const idlFactory = ({ IDL }) => {
  const TxError = IDL.Variant({
    'InsufficientAllowance' : IDL.Null,
    'InsufficientBalance' : IDL.Null,
    'ErrorOperationStyle' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'LedgerTrap' : IDL.Null,
    'ErrorTo' : IDL.Null,
    'Other' : IDL.Text,
    'BlockUsed' : IDL.Null,
    'AmountTooSmall' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat16, 'Err' : TxError });
  return IDL.Service({
    'transferFrom' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Nat],
        [Result],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
