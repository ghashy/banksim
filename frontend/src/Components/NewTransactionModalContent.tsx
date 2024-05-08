import styles from "./ModalWindow.module.scss";
import { FC, FormEvent, useEffect, useState } from "react";
import { IoChevronDownOutline } from "react-icons/io5";
import { format_price, wait } from "../helpers";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import { FetchError, IAccount } from "../types";
import useAxios from "../hooks/useAxios";
import { API_URL, AUTH_HEADER, MAX_RETRIES, RETRY_DELAY_MS } from "../config";
import ErrorModalContent from "./ErrorModalContent";

interface NewTransactionModalContentProps {
  hide_window: () => void;
}

interface FormData {
  from: string;
  to: string;
  amount: number | null;
}

const NewTransactionModalContent: FC<NewTransactionModalContentProps> = ({
  hide_window,
}) => {
  const [form_data, set_form_data] = useState<FormData>({
    from: "",
    to: "",
    amount: null,
  });
  const [button_disabled, set_button_disabled] = useState(true);
  const [fetching, set_fetching] = useState(false);
  const [fetch_error, set_fetch_error] = useState<FetchError>({
    message: "",
  });
  const account_list = useSelector<RootState, IAccount[]>(
    (state) => state.account_list.account_list
  );
  const store_card = useSelector<RootState, string>(
    (state) => state.store_info.card.content
  );
  const { fetch_data: new_transaction } = useAxios();

  function handle_change(
    e: React.ChangeEvent<HTMLSelectElement | HTMLInputElement>
  ) {
    const { name, value } = e.target;
    let only_digit = value.replace(/[^\d]/g, "");

    if (name === "amount") {
      set_form_data((prev) => ({
        ...prev,
        amount: parseInt(only_digit),
      }));
      return;
    }

    set_form_data((prev) => ({
      ...prev,
      [name]: value,
    }));
  }

  async function handle_submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    if (fetching) {
      return;
    }

    const data = JSON.stringify(form_data);
    let attempts = 1;

    recursive_call(data, attempts);
  }

  async function recursive_call(data: string, attempts: number) {
    set_fetching(true);
    const response = await new_transaction({
      method: "POST",
      url: `${API_URL}/system/transaction`,
      headers: {
        Authorization: AUTH_HEADER,
        "Content-Type": "application/json",
      },
      data: data,
    });

    if (response.ok) {
      hide_window();
    } else if (
      response.err &&
      response.val.recursive &&
      attempts < MAX_RETRIES
    ) {
      attempts++;
      await wait(RETRY_DELAY_MS);
      recursive_call(data, attempts);
    } else {
      set_fetching(false);
      set_fetch_error({
        message: response.val.message,
        err_status: response.val.err_status,
        recursive: response.val.recursive,
      });
    }
  }

  function retry_fetch() {
    if (fetch_error.recursive) {
      recursive_call(JSON.stringify(form_data), 1);
    } else {
      set_fetch_error({
        message: "",
        err_status: undefined,
        recursive: undefined,
      });
      set_form_data((prev) => ({
        ...prev,
        amount: null,
      }));
    }
  }

  useEffect(() => {
    if (Object.values(form_data).every((field) => field)) {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

  if (fetch_error.message) {
    return (
      <ErrorModalContent
        error_response_status={fetch_error.err_status}
        error_data={fetch_error.message}
        recursive={fetch_error.recursive}
        fetching={fetching}
        handle_retry={retry_fetch}
      />
    );
  }

  return (
    <>
      <h2 className={styles.h2}>New Transaction</h2>
      <form
        onSubmit={handle_submit}
        className={styles.submit_form}
      >
        <div className={styles.select_container}>
          <IoChevronDownOutline
            className={styles.arrow_down}
            style={{
              color: `${form_data.from ? "#000000" : "#808080"}`,
            }}
          />
          <p className={styles.label}>From</p>
          <select
            name="from"
            id="from"
            value={form_data.from}
            onChange={handle_change}
            className={styles.select}
            style={{
              color: `${form_data.from ? "#000000" : "#808080"}`,
            }}
          >
            <option
              value=""
              disabled
            >
              card number
            </option>
            {form_data.to !== store_card.toString() && (
              <option value={store_card}>{`${store_card} (Store card)`}</option>
            )}
            {account_list
              .filter((account) => account.exists)
              .filter((account) => account.card_number !== form_data.to)
              .map((account, idx) => (
                <option
                  value={account.card_number}
                  key={idx}
                >
                  {`${account.card_number} (${account.username})`}
                </option>
              ))}
          </select>
        </div>
        <div className={styles.select_container}>
          <p className={styles.label}>To</p>
          <IoChevronDownOutline
            className={styles.arrow_down}
            style={{
              color: `${form_data.to ? "#000000" : "#808080"}`,
            }}
          />
          <select
            name="to"
            id="to"
            value={form_data.to}
            onChange={handle_change}
            className={styles.select}
            style={{
              color: `${form_data.to ? "#000000" : "#808080"}`,
            }}
          >
            <option
              value=""
              disabled
            >
              card number
            </option>
            {form_data.from !== store_card.toString() && (
              <option value={store_card}>{`${store_card} (Store card)`}</option>
            )}
            {account_list
              .filter((account) => account.exists)
              .filter((account) => account.card_number !== form_data.from)
              .map((account, idx) => (
                <option
                  value={account.card_number}
                  key={idx}
                >
                  {`${account.card_number} (${account.username})`}
                </option>
              ))}
          </select>
        </div>
        <label
          htmlFor="amount"
          className={styles.label}
        >
          Amount
        </label>
        <input
          type="text"
          id="amount"
          name="amount"
          placeholder="10_000"
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

export default NewTransactionModalContent;
