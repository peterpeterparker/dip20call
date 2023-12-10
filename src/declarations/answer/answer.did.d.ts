import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Result = { 'Ok' : number } |
  { 'Err' : TxError };
export type TxError = { 'InsufficientAllowance' : null } |
  { 'InsufficientBalance' : null } |
  { 'ErrorOperationStyle' : null } |
  { 'Unauthorized' : null } |
  { 'LedgerTrap' : null } |
  { 'ErrorTo' : null } |
  { 'Other' : string } |
  { 'BlockUsed' : null } |
  { 'AmountTooSmall' : null };
export interface _SERVICE {
  'transferFrom' : ActorMethod<[Principal, Principal, bigint], Result>,
}
