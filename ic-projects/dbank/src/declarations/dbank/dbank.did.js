export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'compound' : IDL.Func([], [], ['oneway']),
    'getBalance' : IDL.Func([], [IDL.Float64], ['query']),
    'toUp' : IDL.Func([IDL.Float64], [], ['oneway']),
    'withDrawl' : IDL.Func([IDL.Float64], [], ['oneway']),
  });
};
export const init = ({ IDL }) => { return []; };
