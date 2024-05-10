import Ansi from "ansi-to-react";
import styles from "./LogsPage.module.scss";
import { FC, useState } from "react";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import { wait } from "../helpers";
import { RETRY_DELAY_MS } from "../config";
import { SocketEndpoints } from "../types";

interface LogsPageProps {
  connect_to_socket: (endpoint: SocketEndpoints) => Promise<void>;
}

const LogsPage: FC<LogsPageProps> = ({ connect_to_socket }) => {
  const logs_socket_open = useSelector<RootState, boolean>(
    (state) => state.socket_open.logs_open
  );
  const logs = useSelector<RootState, string[]>((state) => state.logs.logs);
  const [fetching, set_fetching] = useState(false);

  async function handle_reconnect_click() {
    if (fetching) {
      return;
    }

    set_fetching(true);

    await wait(RETRY_DELAY_MS);
    await connect_to_socket("subscribe_on_traces");

    set_fetching(false);
  }

  return (
    <section className={styles.logs_page}>
      <div className={styles.content}>
        <h2 className={styles.h2}>Logs</h2>
        {!logs_socket_open ? (
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
        ) : (
          <div className={styles.logs_container}>
            {logs.map((log, idx) => {
              return (
                <div
                  key={idx}
                  className={styles.log_message}
                >
                  <Ansi>{log}</Ansi>
                  <br />
                </div>
              );
            })}
          </div>
        )}
      </div>
    </section>
  );
};

export default LogsPage;
