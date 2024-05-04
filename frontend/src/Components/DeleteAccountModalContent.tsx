import styles from "./ModalWindow.module.scss";
import { useSelector } from "react-redux";
import { FC, useState } from "react";
import { RootState } from "../state/store";
import useAxios from "../hooks/useAxios";
import { API_URL } from "../config";
import { handle_retry } from "../helpers";
import ErrorModalContent from "./ErrorModalContent";

interface DeleteAccountModalContentProps {
  hide_window: () => void;
}

const DeleteAccountModalContent: FC<DeleteAccountModalContentProps> = ({
  hide_window,
}) => {
  const card_numbers = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  ).filter((card_number) => card_number !== "01");
  const [fetching, set_fetching] = useState(false);
  const {
    fetch_data: delete_account,
    error_data: error_data,
    set_error_data: set_error_data,
    response_status: error_response_status,
    set_response_status: set_error_response_status,
  } = useAxios();

  function handle_delete() {
    if (fetching) {
      return;
    }

    set_fetching(true);

    async function remove(card_number: string) {
      const data = JSON.stringify({ card_number: card_number });

      const response = await delete_account({
        method: "DELETE",
        url: `${API_URL}/system/account`,
        headers: {
          "Content-Type": "application/json",
        },
        data: data,
      });

      return response;
    }

    const requests = card_numbers.map((card_number) => remove(card_number));

    Promise.all(requests)
      .then((response: any[]) => {
        if (response.every((answer) => answer)) {
          hide_window();
        }
      })
      .catch((err) => console.error(err));
  }

  function set_states() {
    set_fetching(false);
    set_error_data("");
    set_error_response_status(0);
  }

  if (error_data) {
    return (
      <ErrorModalContent
        error_response_status={error_response_status}
        error_data={error_data}
        handle_retry={() => handle_retry(error_response_status, set_states)}
      />
    );
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
          {fetching ? (
            <span
              className={`${styles.loader_small} ${styles.loader_delete}`}
            ></span>
          ) : (
            "Delete"
          )}
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
