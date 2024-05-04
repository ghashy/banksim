import Ansi from "ansi-to-react";
import styles from "./LogsPage.module.scss";
import { FC } from "react";

const LogsPage: FC = () => {
  return (
    <section className={styles.logs_page}>
      <div className={styles.content}>
        <Ansi>{`1. Error log with timestamp:
\u001b[31m[ERROR] \u001b[37m2022-04-01 13:45:22 \u001b[0mAn unexpected error occurred: Connection timed out`}</Ansi>
        <br />
        <Ansi>{`2. Warning log with detailed message:
\u001b[33m[WARNING] \u001b[37m[3145] \u001b[0mPossible data inconsistency detected in user records`}</Ansi>
        <br />
        <Ansi>{`3. Info log with request details:
\u001b[36m[INFO] \u001b[37mGET /api/users/1234 \u001b[0mRequest successfully processed in 50ms`}</Ansi>
        <br />
      </div>
    </section>
  );
};

export default LogsPage;
