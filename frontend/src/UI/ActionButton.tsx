import styles from "./ActionButton.module.scss";
import { FC, useEffect, useState } from "react";
import { ActionButtonKind } from "../types";
import { AiOutlineDollarCircle } from "react-icons/ai";
import { LuPlusCircle } from "react-icons/lu";
import { IoTrash } from "react-icons/io5";
import { TbCreditCardPay } from "react-icons/tb";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";

interface ActionButtonProps {
  kind: ActionButtonKind;
}

const ActionButton: FC<ActionButtonProps> = ({ kind }) => {
  const checked_items = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  );
  const [button_name, set_button_name] = useState("");

  useEffect(() => {
    switch (kind) {
      case "new_account":
        set_button_name("new account");
        break;
      case "new_transaction":
        set_button_name("new transaction");
        break;
      case "delete_account":
        set_button_name("delete account");
        break;
      case "open_credit":
        set_button_name("open credit");
        break;
    }
  }, [kind]);

  useEffect(() => {
    if (kind === "delete_account") {
      if (checked_items.length <= 1) {
        set_button_name("delete account");
      } else {
        set_button_name("delete accounts");
      }
    }
  }, [checked_items.length]);

  return (
    <div className={styles.action_button}>
      <p className={styles.button_name}>{button_name}</p>
      {kind === "new_account" && (
        <LuPlusCircle className={styles.button_icon} />
      )}
      {kind === "new_transaction" && (
        <AiOutlineDollarCircle className={styles.button_icon} />
      )}
      {kind === "open_credit" && (
        <TbCreditCardPay className={styles.button_icon} />
      )}
      {kind === "delete_account" && <IoTrash className={styles.button_icon} />}
    </div>
  );
};

export default ActionButton;
