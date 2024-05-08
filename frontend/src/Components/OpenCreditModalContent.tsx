import styles from "./ModalWindow.module.scss";
import { useDispatch, useSelector } from "react-redux";
import { RootState } from "../state/store";
import { FC, FormEvent, useEffect, useState } from "react";
import { format_price } from "../helpers";
import useAxios from "../hooks/useAxios";
import { API_URL, AUTH_HEADER } from "../config";
import { reset_checked_itmes } from "../state/checked_items_slice";

interface OpenCreditModalContentProps {
  hide_window: () => void;
}

interface FormData {
  amount: number | null;
}

const OpenCreditModalContent: FC<OpenCreditModalContentProps> = ({
  hide_window,
}) => {
  const [form_data, set_form_data] = useState<FormData>({
    amount: null,
  });
  const [button_disabled, set_button_disabled] = useState(true);
  const [fetching, set_fetching] = useState(false);
  const [fetching_info_visible, set_fetching_info_visible] = useState(false);
  const [fetch_info, set_fetch_info] = useState<string[]>([]);
  const card_numbers = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  ).filter((card_number) => card_number !== "01");
  const { fetch_data: open_credit } = useAxios();
  const dispatch = useDispatch();

  function handle_change(e: React.ChangeEvent<HTMLInputElement>) {
    const value = e.target.value;
    let only_digit = value.replace(/[^\d]/g, "");

    set_form_data({
      amount: parseInt(only_digit),
    });
  }

  async function handle_submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    if (fetching) {
      return;
    }

    async function credit(card_number: string) {
      const data = JSON.stringify({
        card_number: card_number,
        amount: form_data.amount,
      });

      const response = await open_credit({
        method: "POST",
        url: `${API_URL}/system/credit`,
        headers: {
          Authorization: AUTH_HEADER,
          "Content-Type": "application/json",
        },
        data: data,
      });

      return response;
    }

    const requests = card_numbers.map((card_number) => credit(card_number));

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

  useEffect(() => {
    if (form_data.amount) {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

  return (
    <>
      <h2 className={styles.h2}>
        Open {card_numbers.length === 1 ? "Credit" : "Credits"}
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
                  Credit to {card_number}:{" "}
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
        <form
          onSubmit={handle_submit}
          className={styles.submit_form}
        >
          <p
            className={styles.info_message}
            style={{
              marginBottom: "2rem",
            }}
          >
            How much money do you want to credit?
          </p>
          <input
            type="text"
            name="password"
            autoFocus
            value={form_data.amount ? format_price(form_data.amount) : ""}
            className={styles.text_input}
            onChange={handle_change}
          />
          <button
            type="submit"
            className={styles.submit_button}
            disabled={button_disabled}
          >
            {fetching ? (
              <span className={styles.loader_small}></span>
            ) : (
              "Submit"
            )}
          </button>
        </form>
      )}
    </>
  );
};

export default OpenCreditModalContent;
