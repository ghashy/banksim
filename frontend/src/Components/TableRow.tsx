import styles from "./AccountTable.module.scss";
import CustomCheckbox from "../UI/CustomCheckbox";
import { IAccount } from "../types";
import { FC, useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import { format_price } from "../helpers";

interface TableRowProps {
  props: IAccount;
}

const TableRow: FC<TableRowProps> = ({ props }) => {
  const [row_class_names, set_row_class_names] = useState(
    `${styles.table_row} ${!props.exists && styles.row_disabled}`
  );
  const checked_items = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  );

  useEffect(() => {
    if (checked_items.includes(props.card_number)) {
      set_row_class_names(`${styles.table_row} ${styles.row_selected}`);
    } else {
      set_row_class_names(
        `${styles.table_row} ${!props.exists && styles.row_disabled}`
      );
    }
  }, [checked_items, props.exists]);

  return (
    <label
      htmlFor={props.card_number}
      className={row_class_names}
    >
      <CustomCheckbox
        card_number={props.card_number}
        disabled={!props.exists}
      />
      <p className={`${styles.table_column} ${styles.card_number}`}>
        <span className={styles.column_content}>{props.card_number}</span>
      </p>
      <p className={`${styles.table_column} ${styles.transactions}`}>
        <span className={styles.column_content}>
          {props.transactions.length}
        </span>
      </p>
      <p className={`${styles.table_column} ${styles.balance}`}>
        {format_price(props.balance)}
      </p>
      <p className={`${styles.table_column} ${styles.username}`}>
        <span className={styles.column_content}>{props.username}</span>
      </p>
      <p className={`${styles.table_column} ${styles.tokens}`}>
        <span className={styles.column_content}>
          {props.tokens.length === 0 ? "No tokens" : "Some tokens"}
        </span>
      </p>
      <p className={`${styles.table_column} ${styles.exists}`}>
        <span className={styles.column_content}>
          {props.exists ? "True" : "False"}
        </span>
      </p>
    </label>
  );
};

export default TableRow;
