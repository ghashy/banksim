import styles from "./AccountTable.module.scss";
import { FC } from "react";
import TableHeader from "./TableHeader";
import TableRow from "./TableRow";
import { accounts } from "../mock_data";

const AccountTable: FC = () => {
  return (
    <div className={styles.account_table}>
      <TableHeader />
      <div className={styles.rows_container}>
        {accounts.map((account, idx) => (
          <TableRow
            props={account}
            key={idx}
          />
        ))}
      </div>
    </div>
  );
};

export default AccountTable;
