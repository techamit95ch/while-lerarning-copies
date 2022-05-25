import type { Principal } from '@dfinity/principal';
export interface _SERVICE {
  'compound' : () => Promise<undefined>,
  'getBalance' : () => Promise<number>,
  'toUp' : (arg_0: number) => Promise<undefined>,
  'withDrawl' : (arg_0: number) => Promise<undefined>,
}
