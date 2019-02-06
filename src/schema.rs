table! {
    block (id) {
        id -> Int4,
        hash -> Nullable<Varchar>,
        strippedsize -> Nullable<Int4>,
        size -> Nullable<Int4>,
        height -> Nullable<Int4>,
        weight -> Nullable<Int4>,
        version -> Nullable<Int4>,
        merkleroot -> Nullable<Varchar>,
        witnessroot -> Nullable<Varchar>,
        filterroot -> Nullable<Varchar>,
        reservedroot -> Nullable<Varchar>,
        nonce -> Nullable<Varchar>,
        time -> Nullable<Int8>,
        bits -> Nullable<Int8>,
        chainwork -> Nullable<Varchar>,
        previousblockhash -> Nullable<Varchar>,
        nextblockhash -> Nullable<Varchar>,
        mined_by -> Nullable<Varchar>,
        total_txs -> Nullable<Int8>,
        fees -> Nullable<Int8>,
        reward -> Nullable<Int8>,
    }
}

table! {
    covenant (id) {
        id -> Int4,
        action -> Nullable<Varchar>,
        name_hash -> Nullable<Varchar>,
        data -> Nullable<Text>,
        nonce -> Nullable<Varchar>,
        output_id -> Nullable<Int8>,
    }
}

table! {
    input (id) {
        id -> Int4,
        tx_id -> Nullable<Int8>,
        index -> Nullable<Int8>,
        address -> Nullable<Varchar>,
        prevout_hash -> Nullable<Varchar>,
        prevout_index -> Nullable<Int8>,
        witness -> Nullable<Text>,
        sequence -> Nullable<Int8>,
        prevout_tx_id -> Nullable<Int8>,
    }
}

table! {
    knex_migrations (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        batch -> Nullable<Int4>,
        migration_time -> Nullable<Timestamptz>,
    }
}

table! {
    knex_migrations_lock (index) {
        index -> Int4,
        is_locked -> Nullable<Int4>,
    }
}

table! {
    output (id) {
        id -> Int4,
        tx_id -> Nullable<Int8>,
        index -> Nullable<Int4>,
        value -> Nullable<Int8>,
        address -> Nullable<Varchar>,
    }
}

table! {
    tx (id) {
        id -> Int4,
        hash -> Nullable<Varchar>,
        txid -> Nullable<Varchar>,
        witnesshash -> Nullable<Varchar>,
        blockhash -> Nullable<Varchar>,
        size -> Nullable<Int8>,
        value -> Nullable<Int8>,
        minFee -> Nullable<Int8>,
        locktime -> Nullable<Int8>,
        fee -> Nullable<Int8>,
        rate -> Nullable<Int8>,
        height -> Nullable<Int8>,
        time -> Nullable<Int8>,
        index -> Nullable<Int8>,
        version -> Nullable<Int8>,
    }
}

//TODO: get these working
// joinable!(input -> tx (tx_id));
// joinable!(output -> tx (tx_id));

allow_tables_to_appear_in_same_query!(
    block,
    covenant,
    input,
    knex_migrations,
    knex_migrations_lock,
    output,
    tx,
);
