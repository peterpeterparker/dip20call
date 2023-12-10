dfx start
dfx deploy answer
(update manually "answer" canister id in "caller" code)
dfx deploy caller
dfx canister call caller hello