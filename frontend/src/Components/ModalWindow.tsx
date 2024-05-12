import styles from "./ModalWindow.module.scss";
import { ActionKind } from "../types";
import { FC, useEffect, useRef, useState } from "react";
import { FaXmark } from "react-icons/fa6";
import NewAccountModalContent from "./NewAccountModalContent";
import NewTransactionModalContent from "./NewTransactionModalContent";
import DeleteAccountModalContent from "./DeleteAccountModalContent";
import OpenCreditModalContent from "./OpenCreditModalContent";
import { useDispatch } from "react-redux";
import { reset_checked_itmes } from "../state/checked_items_slice";

interface ModalWindowProps {
  kind: ActionKind;
  hide_window: () => void;
}

const ModalWindow: FC<ModalWindowProps> = ({ kind, hide_window }) => {
  const content_ref = useRef<HTMLDivElement>(null);
  const [content_kind_class_name, set_content_kind_class_name] = useState("");
  const dispatch = useDispatch();

  function define_class_name(kind: ActionKind): void {
    switch (kind) {
      case "":
        set_content_kind_class_name("");
        break;
      case "new_account":
        set_content_kind_class_name(`${styles.content_new_account}`);
        break;
      case "new_transaction":
        set_content_kind_class_name(`${styles.content_new_transaction}`);
        break;
      case "delete_account":
        set_content_kind_class_name(`${styles.content_delete_account}`);
        break;
      case "open_credit":
        set_content_kind_class_name(`${styles.content_open_credit}`);
        break;
    }
  }

  function handle_modal_close() {
    dispatch(reset_checked_itmes());
    hide_window();
  }

  useEffect(() => {
    define_class_name(kind);
  }, []);

  useEffect(() => {
    const handle_click_outside = (e: MouseEvent) => {
      if (content_ref.current) {
        if (!content_ref.current.contains(e.target as Node)) {
          hide_window();
        }
      }
    };

    document.addEventListener("mousedown", handle_click_outside);

    return () => {
      document.removeEventListener("mousedown", handle_click_outside);
    };
  }, []);

  useEffect(() => {
    const handle_esc_press = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        handle_modal_close();
      }
    };

    document.addEventListener("keydown", handle_esc_press);

    return () => {
      document.removeEventListener("keydown", handle_esc_press);
    };
  }, []);
  return (
    <div className={styles.modal_bg}>
      <div
        ref={content_ref}
        className={`${styles.content} ${content_kind_class_name}`}
      >
        <FaXmark
          className={styles.close_icon}
          onClick={handle_modal_close}
        />
        {kind === "new_transaction" && (
          <NewTransactionModalContent hide_window={hide_window} />
        )}
        {kind === "new_account" && (
          <NewAccountModalContent hide_window={hide_window} />
        )}
        {kind === "open_credit" && (
          <OpenCreditModalContent hide_window={hide_window} />
        )}
        {kind === "delete_account" && (
          <DeleteAccountModalContent hide_window={hide_window} />
        )}
      </div>
    </div>
  );
};

export default ModalWindow;
