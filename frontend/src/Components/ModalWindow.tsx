import styles from "./ModalWindow.module.scss";
import { ActionKind } from "../types";
import { FC, useEffect, useRef } from "react";
import { FaXmark } from "react-icons/fa6";
import NewAccountModalContent from "./NewAccountModalContent";
import NewTransactionModalContent from "./NewTransactionModalContent";
import DeleteAccountModalContent from "./DeleteAccountModalContent";

interface ModalWindowProps {
  kind: ActionKind;
  hide_window: () => void;
}

const ModalWindow: FC<ModalWindowProps> = ({ kind, hide_window }) => {
  const content_ref = useRef<HTMLDivElement>(null);

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
        hide_window();
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
        className={styles.content}
      >
        <FaXmark
          className={styles.close_icon}
          onClick={hide_window}
        />
        {kind === "new_transaction" && <NewTransactionModalContent />}
        {kind === "new_account" && <NewAccountModalContent />}
        {kind === "open_credit" && <div>open credit</div>}
        {kind === "delete_account" && (
          <DeleteAccountModalContent hide_window={hide_window} />
        )}
      </div>
    </div>
  );
};

export default ModalWindow;
