import styles from "./ModalWindow.module.scss";
import { FC } from "react";

interface ErrorModalContentProps {
  error_response_status: number;
  error_data: string;
  handle_retry: () => void;
}

const ErrorModalContent: FC<ErrorModalContentProps> = ({
  error_response_status,
  error_data,
  handle_retry,
}) => {
  return (
    <>
      <h2 className={styles.h2}>
        Error {error_response_status ? error_response_status : "no response"}
      </h2>
      <p className={styles.info_message}>{error_data}</p>
      <div
        className={`${styles.button} ${styles.retry_button}`}
        onClick={handle_retry}
      >
        Retry
      </div>
    </>
  );
};

export default ErrorModalContent;
