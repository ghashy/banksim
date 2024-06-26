import CustomCheckbox from "../UI/CustomCheckbox";
import styles from "./AccountTable.module.scss";
import { FC } from "react";

const TableHeader: FC = () => {
  return (
    <div className={styles.table_header}>
      <CustomCheckbox
        card_number="01"
        disabled={false}
      />
      <p className={`${styles.table_column} ${styles.card_number}`}>
        Card number
      </p>
      <p className={`${styles.table_column} ${styles.transactions}`}>
        Transactions
      </p>
      <p className={`${styles.table_column} ${styles.balance}`}>Balance</p>
      <p className={`${styles.table_column} ${styles.username}`}>Username</p>
      <p className={`${styles.table_column} ${styles.tokens}`}>Tokens</p>
      <p className={`${styles.table_column} ${styles.exists}`}>Exists</p>
    </div>
  );
};

export default TableHeader;
