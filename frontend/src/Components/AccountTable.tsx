import styles from "./AccountTable.module.scss";
import { FC } from "react";
import TableHeader from "./TableHeader";
import TableRow from "./TableRow";
import { accounts } from "../mock_data";
import { RootState } from "../state/store";
import { useDispatch, useSelector } from "react-redux";
import { reset_checked_itmes } from "../state/checked_items_slice";

const AccountTable: FC = () => {
  const checked_items = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  );
  const dispatch = useDispatch();

  return (
    <div className={styles.account_table}>
      <TableHeader />
      <div className={styles.rows_container}>
        {accounts.map((account, idx) => (
          <TableRow
            props={account}
            idx={idx}
            key={idx}
          />
        ))}
      </div>
      <div
        className={styles.unselect_all}
        style={{
          visibility: `${checked_items.length !== 0 ? "visible" : "hidden"}`,
        }}
        onClick={() => dispatch(reset_checked_itmes())}
      >
        unselect all
      </div>
    </div>
  );
};

export default AccountTable;
