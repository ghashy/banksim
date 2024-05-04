import styles from "./AccountTable.module.scss";
import { FC, useEffect, useState } from "react";
import TableHeader from "./TableHeader";
import TableRow from "./TableRow";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import { IAccount } from "../types";
import TableSkeleton from "./TableSkeleton";

const AccountTable: FC = () => {
  const account_list = useSelector<RootState, IAccount[]>(
    (state) => state.account_list.account_list
  );
  const accounts_loading = useSelector<RootState, boolean>(
    (state) => state.account_list.is_loading
  );
  const [sorted_list, set_sorted_list] = useState<IAccount[]>([]);

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
      {accounts_loading ? (
        <TableSkeleton />
      ) : (
        <div className={styles.rows_container}>
          {sorted_list.map((account, idx) => (
            <TableRow
              props={account}
              key={idx}
            />
          ))}
        </div>
      )}
    </div>
  );
};

export default AccountTable;
