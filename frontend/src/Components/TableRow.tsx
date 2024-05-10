import styles from "./AccountTable.module.scss";
import CustomCheckbox from "../UI/CustomCheckbox";
import { IAccount } from "../types";
import { FC, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import { format_price } from "../helpers";
import { IoChevronDownOutline } from "react-icons/io5";

interface TableRowProps {
  props: IAccount;
  show_tokens: (card_number: string) => void;
  visible_tokens: string;
}

const TableRow: FC<TableRowProps> = ({
  props,
  show_tokens,
  visible_tokens,
}) => {
  const [row_class_names, set_row_class_names] = useState(
    `${styles.table_row} ${!props.exists && styles.row_disabled}`
  );
  const tokens_container_ref = useRef<HTMLDivElement>(null);
  const tokens_button = useRef<HTMLDivElement>(null);
  const checked_items = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  );

  function handle_tokens_click(e: React.MouseEvent<HTMLDivElement>) {
    e.preventDefault();
    e.stopPropagation();
    show_tokens(props.card_number);
  }

  useEffect(() => {
    const handle_click_outside = (e: MouseEvent) => {
      if (tokens_container_ref.current && tokens_button.current) {
        if (
          !tokens_container_ref.current.contains(e.target as Node) &&
          !tokens_button.current.contains(e.target as Node)
        ) {
          show_tokens("");
        }
      }
    };

    document.addEventListener("mousedown", handle_click_outside);

    return () => {
      document.removeEventListener("mousedown", handle_click_outside);
    };
  }, []);

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
      <div className={`${styles.table_column} ${styles.tokens}`}>
        {props.tokens.length !== 0 ? (
          <div
            ref={tokens_button}
            className={`${styles.show_tokens_button} ${
              visible_tokens === props.card_number &&
              styles.tokens_button_active
            }`}
            onClick={handle_tokens_click}
          >
            Show tokens
            <IoChevronDownOutline className={styles.chevron} />
          </div>
        ) : (
          <div className={styles.no_tokens}>No tokens</div>
        )}
        {visible_tokens === props.card_number && (
          <div
            ref={tokens_container_ref}
            className={styles.tokens_container}
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
            }}
          >
            {props.tokens.map((token, idx) => {
              return (
                <p
                  key={idx}
                  className={styles.token}
                >
                  {token}
                </p>
              );
            })}
          </div>
        )}
      </div>
      <p className={`${styles.table_column} ${styles.exists}`}>
        <span className={styles.column_content}>
          {props.exists ? "True" : "False"}
        </span>
      </p>
    </label>
  );
};

export default TableRow;
