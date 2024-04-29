import styles from "./AccountTable.module.scss";
import CustomCheckbox from "../UI/CustomCheckbox";
import { IAccount } from "../types";
import { FC, useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";

interface TableRowProps {
  props: IAccount;
  idx: number;
}

function format_price(price: number): string {
  return price.toLocaleString().replace(/,/g, "_");
}

const TableRow: FC<TableRowProps> = ({ props, idx }) => {
  const checked_items = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  );
  const [row_class_names, set_row_class_names] = useState(
    `${styles.table_row}`
  );

  useEffect(() => {
    if (checked_items.includes(`row${idx}`)) {
      set_row_class_names(`${styles.table_row} ${styles.row_selected}`);
    } else {
      set_row_class_names(`${styles.table_row}`);
    }
  }, [checked_items]);

  return (
    <div className={row_class_names}>
      <CustomCheckbox name={`row${idx}`} />
      <p className={`${styles.table_column} ${styles.card_number}`}>
        {props.card_number}
      </p>
      <p className={`${styles.table_column} ${styles.transactions}`}>
        {props.transactions}
      </p>
      <p className={`${styles.table_column} ${styles.balance}`}>
        {format_price(props.balance)}
      </p>
      <p className={`${styles.table_column} ${styles.username}`}>
        {props.username}
      </p>
      <p className={`${styles.table_column} ${styles.tokens}`}>
        {props.tokens[0]}
      </p>
      <p className={`${styles.table_column} ${styles.exists}`}>
        {props.exists ? "True" : "False"}
      </p>
    </div>
  );
};

export default TableRow;