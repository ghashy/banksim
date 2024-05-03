import styles from "./AccountTable.module.scss";
import { FC } from "react";
import TableHeader from "./TableHeader";
import TableRow from "./TableRow";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import { IAccount } from "../types";

const AccountTable: FC = () => {
  const account_list = useSelector<RootState, IAccount[]>(
    (state) => state.account_list.account_list
  );

  return (
    <div className={styles.account_table}>
      <TableHeader />
      <div className={styles.rows_container}>
        {account_list.map((account, idx) => (
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
