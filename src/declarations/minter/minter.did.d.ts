import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export type BitcoinAddress = { 'p2sh' : Uint8Array | number[] } |
  { 'p2wpkh_v0' : Uint8Array | number[] } |
  { 'p2pkh' : Uint8Array | number[] };
export type BtcNetwork = { 'Mainnet' : null } |
  { 'Regtest' : null } |
  { 'Testnet' : null };
export type Event = {
    'received_utxos' : { 'to_account' : Account, 'utxos' : Array<Utxo> }
  } |
  {
    'sent_transaction' : {
      'change_output' : [] | [{ 'value' : bigint, 'vout' : number }],
      'txid' : Uint8Array | number[],
      'utxos' : Array<Utxo>,
      'requests' : BigUint64Array | bigint[],
      'submitted_at' : bigint,
    }
  } |
  {
    'distributed_kyt_fee' : {
      'block_index' : bigint,
      'amount' : bigint,
      'kyt_provider' : Principal,
    }
  } |
  { 'init' : InitArgs } |
  { 'upgrade' : UpgradeArgs } |
  {
    'retrieve_btc_kyt_failed' : {
      'block_index' : bigint,
      'uuid' : string,
      'address' : string,
      'amount' : bigint,
      'kyt_provider' : Principal,
    }
  } |
  {
    'accepted_retrieve_btc_request' : {
      'received_at' : bigint,
      'block_index' : bigint,
      'address' : BitcoinAddress,
      'amount' : bigint,
      'kyt_provider' : [] | [Principal],
    }
  } |
  {
    'checked_utxo' : {
      'clean' : boolean,
      'utxo' : Utxo,
      'uuid' : string,
      'kyt_provider' : [] | [Principal],
    }
  } |
  { 'removed_retrieve_btc_request' : { 'block_index' : bigint } } |
  { 'confirmed_transaction' : { 'txid' : Uint8Array | number[] } } |
  { 'ignored_utxo' : { 'utxo' : Utxo } };
export interface InitArgs {
  'kyt_principal' : [] | [Principal],
  'ecdsa_key_name' : string,
  'mode' : Mode,
  'retrieve_btc_min_amount' : bigint,
  'ledger_id' : Principal,
  'max_time_in_queue_nanos' : bigint,
  'btc_network' : BtcNetwork,
  'min_confirmations' : [] | [number],
  'kyt_fee' : [] | [bigint],
}
export type MinterArg = { 'Upgrade' : [] | [UpgradeArgs] } |
  { 'Init' : InitArgs };
export interface MinterInfo {
  'retrieve_btc_min_amount' : bigint,
  'min_confirmations' : number,
  'kyt_fee' : bigint,
}
export type Mode = { 'RestrictedTo' : Array<Principal> } |
  { 'DepositsRestrictedTo' : Array<Principal> } |
  { 'ReadOnly' : null } |
  { 'GeneralAvailability' : null };
export interface RetrieveBtcArgs { 'address' : string, 'amount' : bigint }
export type RetrieveBtcError = { 'MalformedAddress' : string } |
  { 'GenericError' : { 'error_message' : string, 'error_code' : bigint } } |
  { 'TemporarilyUnavailable' : string } |
  { 'AlreadyProcessing' : null } |
  { 'AmountTooLow' : bigint } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface RetrieveBtcOk { 'block_index' : bigint }
export type RetrieveBtcStatus = { 'Signing' : null } |
  { 'Confirmed' : { 'txid' : Uint8Array | number[] } } |
  { 'Sending' : { 'txid' : Uint8Array | number[] } } |
  { 'AmountTooLow' : null } |
  { 'Unknown' : null } |
  { 'Submitted' : { 'txid' : Uint8Array | number[] } } |
  { 'Pending' : null };
export type UpdateBalanceError = {
    'GenericError' : { 'error_message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : string } |
  { 'AlreadyProcessing' : null } |
  {
    'NoNewUtxos' : {
      'required_confirmations' : number,
      'current_confirmations' : [] | [number],
    }
  };
export interface UpgradeArgs {
  'kyt_principal' : [] | [Principal],
  'mode' : [] | [Mode],
  'retrieve_btc_min_amount' : [] | [bigint],
  'max_time_in_queue_nanos' : [] | [bigint],
  'min_confirmations' : [] | [number],
  'kyt_fee' : [] | [bigint],
}
export interface Utxo {
  'height' : number,
  'value' : bigint,
  'outpoint' : { 'txid' : Uint8Array | number[], 'vout' : number },
}
export type UtxoStatus = { 'ValueTooSmall' : Utxo } |
  { 'Tainted' : Utxo } |
  {
    'Minted' : {
      'minted_amount' : bigint,
      'block_index' : bigint,
      'utxo' : Utxo,
    }
  } |
  { 'Checked' : Utxo };
export interface _SERVICE {
  'estimate_withdrawal_fee' : ActorMethod<
    [{ 'amount' : [] | [bigint] }],
    { 'minter_fee' : bigint, 'bitcoin_fee' : bigint }
  >,
  'get_btc_address' : ActorMethod<
    [
      {
        'owner' : [] | [Principal],
        'subaccount' : [] | [Uint8Array | number[]],
      },
    ],
    string
  >,
  'get_deposit_fee' : ActorMethod<[], bigint>,
  'get_events' : ActorMethod<
    [{ 'start' : bigint, 'length' : bigint }],
    Array<Event>
  >,
  'get_minter_info' : ActorMethod<[], MinterInfo>,
  'get_withdrawal_account' : ActorMethod<[], Account>,
  'retrieve_btc' : ActorMethod<
    [RetrieveBtcArgs],
    { 'Ok' : RetrieveBtcOk } |
      { 'Err' : RetrieveBtcError }
  >,
  'retrieve_btc_status' : ActorMethod<
    [{ 'block_index' : bigint }],
    RetrieveBtcStatus
  >,
  'update_balance' : ActorMethod<
    [
      {
        'owner' : [] | [Principal],
        'subaccount' : [] | [Uint8Array | number[]],
      },
    ],
    { 'Ok' : Array<UtxoStatus> } |
      { 'Err' : UpdateBalanceError }
  >,
}
