import styles from "./ModalWindow.module.scss";
import { useDispatch, useSelector } from "react-redux";
import { FC, useState } from "react";
import { RootState } from "../state/store";
import useAxios from "../hooks/useAxios";
import { API_URL, AUTH_HEADER } from "../config";
import { reset_checked_itmes } from "../state/checked_items_slice";

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
  const [fetching_info_visible, set_fetching_info_visible] = useState(false);
  const [fetch_info, set_fetch_info] = useState<string[]>([]);
  const { fetch_data: delete_account } = useAxios();
  const dispatch = useDispatch();

  async function handle_delete() {
    if (fetching) {
      return;
    }

    async function remove(card_number: string) {
      const data = JSON.stringify({ card_number: card_number });

      const response = await delete_account({
        method: "DELETE",
        url: `${API_URL}/system/account`,
        headers: {
          Authorization: AUTH_HEADER,
          "Content-Type": "application/json",
        },
        data: data,
      });

      return response;
    }

    const requests = card_numbers.map((card_number) => remove(card_number));

    set_fetching(true);
    set_fetching_info_visible(true);

    for await (const response of requests) {
      if (response.ok) {
        set_fetch_info((prev) => [...prev, `Success`]);
      } else {
        set_fetch_info((prev) => [...prev, `Fail`]);
      }
    }

    set_fetching(false);
  }
  function handle_return() {
    dispatch(reset_checked_itmes());
    hide_window();
  }

  return (
    <>
      <h2 className={styles.h2}>
        Delete {card_numbers.length === 1 ? "Account" : "Accounts"}
      </h2>
      {fetching_info_visible ? (
        <>
          <div className={styles.info_container}>
            {card_numbers.map((card_number, idx) => {
              return (
                <p
                  key={idx}
                  className={styles.info_status}
                >
                  Deleting {card_number}:{" "}
                  <span
                    className={`${
                      fetch_info[idx] === "Success" && styles.status_success
                    } ${fetch_info[idx] === "Fail" && styles.status_fail}`}
                  >
                    {fetch_info[idx] ? fetch_info[idx] : "processing..."}
                  </span>
                </p>
              );
            })}
          </div>
          <div
            className={`${styles.button} ${styles.retry_button}`}
            onClick={handle_return}
          >
            {fetching ? (
              <span className={styles.loader_small}></span>
            ) : (
              "Got it"
            )}
          </div>
        </>
      ) : (
        <>
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
      )}
    </>
  );
};

export default DeleteAccountModalContent;
