import styles from "./TransactionsPage.module.scss";
import { FC } from "react";

const TransactionsPage: FC = () => {
  return (
    <section className={styles.transactions_page}>
      <div className={styles.content}>Transactions</div>
    </section>
  );
};

export default TransactionsPage;
