import styles from "./ModalWindow.module.scss";
import { useSelector } from "react-redux";
import { FC } from "react";
import { RootState } from "../state/store";
import useAxios from "../hooks/useAxios";
import { API_URL, AUTH_HEADER } from "../config";

interface DeleteAccountModalContentProps {
  hide_window: () => void;
}

const DeleteAccountModalContent: FC<DeleteAccountModalContentProps> = ({
  hide_window,
}) => {
  const card_numbers = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  ).filter((card_number) => card_number !== "01");
  const { fetch_data: delete_account } = useAxios();

  function handle_delete() {
    card_numbers.forEach(async (number) => {
      const data = JSON.stringify({ card_number: number });

      await delete_account({
        method: "DELETE",
        url: `${API_URL}/system/account`,
        headers: {
          Authorization: AUTH_HEADER,
          "Content-Type": "application/json",
        },
        data: data,
      });

      //TODO Add error handling
    });
    hide_window();
  }

  return (
    <>
      <h2 className={styles.h2}>
        Delete {card_numbers.length === 1 ? "Account" : "Accounts"}
      </h2>
      <p className={styles.info_message}>
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
