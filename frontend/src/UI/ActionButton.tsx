import styles from "./ActionButton.module.scss";
import { FC, useEffect, useState } from "react";
import { ActionButtonKind } from "../types";
import { AiOutlineDollarCircle } from "react-icons/ai";
import { LuPlusCircle } from "react-icons/lu";

interface ActionButtonProps {
  kind: ActionButtonKind;
}

const ActionButton: FC<ActionButtonProps> = ({ kind }) => {
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
  }, []);

  return (
    <div className={styles.action_button}>
      <p className={styles.button_name}>{button_name}</p>
      {kind === "new_account" && (
        <LuPlusCircle className={styles.button_icon} />
      )}
      {kind === "new_transaction" && (
        <AiOutlineDollarCircle className={styles.button_icon} />
      )}
    </div>
  );
};

export default ActionButton;
