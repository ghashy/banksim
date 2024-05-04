import Ansi from "ansi-to-react";
import styles from "./LogsPage.module.scss";
import { FC } from "react";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";

const LogsPage: FC = () => {
  const logs = useSelector<RootState, string[]>((state) => state.logs.logs);
  return (
    <section className={styles.logs_page}>
      <div className={styles.content}>
        <h2 className={styles.h2}>Logs</h2>
        {logs.map((log, idx) => {
          return (
            <div key={idx}>
              <Ansi>{log}</Ansi>
              <br />
            </div>
          );
        })}
      </div>
    </section>
  );
};

export default LogsPage;
