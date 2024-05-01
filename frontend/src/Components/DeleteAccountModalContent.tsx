import styles from "./ModalWindow.module.scss";
import { useSelector } from "react-redux";
import { FC } from "react";
import { RootState } from "../state/store";

interface DeleteAccountModalContentProps {
  hide_window: () => void;
}

const DeleteAccountModalContent: FC<DeleteAccountModalContentProps> = ({
  hide_window,
}) => {
  const card_numbers = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  ).filter((card_number) => card_number !== "01");

  function handle_delete() {
    card_numbers.forEach((number) =>
      console.log(`Account with number ${number} was deleted`)
    );
  }

  return (
    <>
      <h2 className={styles.h2}>Delete Account</h2>
      <p className={styles.warning_message}>
        Are you sure you want to delete{" "}
        {card_numbers.length === 1 ? "an account" : "accounts"}?
      </p>
      <div className={styles.action_buttons}>
        <div
          className={`${styles.button} ${styles.delete_button}`}
          onClick={handle_delete}
        >
          Delete
        </div>
        <div
          className={`${styles.button} ${styles.cancel_button}`}
          onClick={hide_window}
        >
          Cancel
        </div>
      </div>
    </>
  );
};

export default DeleteAccountModalContent;
