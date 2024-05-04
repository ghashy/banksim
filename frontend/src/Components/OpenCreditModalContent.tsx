import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import styles from "./ModalWindow.module.scss";
import { FC, FormEvent, useEffect, useState } from "react";
import { format_price, handle_retry } from "../helpers";
import useAxios from "../hooks/useAxios";
import { API_URL } from "../config";
import ErrorModalContent from "./ErrorModalContent";

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
  const card_numbers = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  ).filter((card_number) => card_number !== "01");
  const {
    fetch_data: open_credit,
    error_data: error_data,
    set_error_data: set_error_data,
    response_status: error_response_status,
    set_response_status: set_error_response_status,
  } = useAxios();

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

    set_fetching(true);

    async function credit(card_number: string) {
      const data = JSON.stringify({
        card_number: card_number,
        amount: form_data.amount,
      });

      const response = await open_credit({
        method: "POST",
        url: `${API_URL}/system/credit`,
        headers: {
          "Content-Type": "application/json",
        },
        data: data,
      });

      return response;
    }

    const requests = card_numbers.map((card_number) => credit(card_number));

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

  useEffect(() => {
    if (form_data.amount) {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

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
        Open {card_numbers.length === 1 ? "Credit" : "Credits"}
      </h2>
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
          {fetching ? <span className={styles.loader_small}></span> : "Submit"}
        </button>
      </form>
    </>
  );
};

export default OpenCreditModalContent;
