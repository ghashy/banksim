import styles from "./AccountTable.module.scss";
import { FC, useEffect, useState } from "react";
import TableHeader from "./TableHeader";
import TableRow from "./TableRow";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import { IAccount, SocketEndpoints } from "../types";
import TableSkeleton from "./TableSkeleton";
import { wait } from "../helpers";
import { RETRY_DELAY_MS } from "../config";

interface AccountTableProps {
  connect_to_socket: (endpoit: SocketEndpoints) => Promise<void>;
}

const AccountTable: FC<AccountTableProps> = ({ connect_to_socket }) => {
  const account_list = useSelector<RootState, IAccount[]>(
    (state) => state.account_list.account_list
  );
  const accounts_loading = useSelector<RootState, boolean>(
    (state) => state.account_list.is_loading
  );
  const accounts_error = useSelector<RootState, string>(
    (state) => state.account_list.error
  );
  const account_socket_open = useSelector<RootState, boolean>(
    (state) => state.socket_open.accounts_open
  );
  const [sorted_list, set_sorted_list] = useState<IAccount[]>([]);
  const [fetching, set_fetching] = useState(false);
  const [visible_tokens, set_visible_tokens] = useState("");

  async function handle_reconnect_click() {
    if (fetching) {
      return;
    }

    set_fetching(true);

    await wait(RETRY_DELAY_MS);
    await connect_to_socket("subscribe_on_accounts");

    set_fetching(false);
  }

  function show_tokens(card_number: string) {
    if (card_number === visible_tokens) {
      set_visible_tokens("");
    } else {
      set_visible_tokens(card_number);
    }
  }

  useEffect(() => {
    set_sorted_list(
      [...account_list].sort((a, b) => {
        if (a.exists && !b.exists) {
          return -1;
        }
        if (!a.exists && b.exists) {
          return 1;
        }
        return 0;
      })
    );
  }, [account_list]);

  return (
    <div className={styles.account_table}>
      <TableHeader />
      {!account_socket_open ? (
        <div className={styles.socket_err}>
          <p>
            {fetching ? (
              <span>
                Reconnecting<span className={styles.dot1}>.</span>
                <span className={styles.dot2}>.</span>
                <span className={styles.dot3}>.</span>
              </span>
            ) : (
              "Websocket connection is lost, try again"
            )}
          </p>
          <div
            onClick={handle_reconnect_click}
            className={styles.reconnect_button}
          >
            {fetching ? (
              <span className={styles.loader_small}></span>
            ) : (
              "Reconnect"
            )}
          </div>
        </div>
      ) : accounts_loading ? (
        <TableSkeleton />
      ) : accounts_error ? (
        <div className={styles.error_message}>{accounts_error}</div>
      ) : (
        <div className={styles.rows_container}>
          {sorted_list.map((account, idx) => (
            <TableRow
              props={account}
              show_tokens={show_tokens}
              visible_tokens={visible_tokens}
              key={idx}
            />
          ))}
        </div>
      )}
    </div>
  );
};

export default AccountTable;
