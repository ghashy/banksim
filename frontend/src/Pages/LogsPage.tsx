import styles from "./LogsPage.module.scss";
import { FC } from "react";

const LogsPage: FC = () => {
  return (
    <section className={styles.logs_page}>
      <div className={styles.content}>Logs</div>
    </section>
  );
};

export default LogsPage;
