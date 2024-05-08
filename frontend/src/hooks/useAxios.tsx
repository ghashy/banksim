import axios, { AxiosRequestConfig, AxiosResponse } from "axios";
import { Ok, Err, Result } from "ts-results";
import { FetchError } from "../types";

const useAxios = () => {
  const fetch_data = async (
    config: AxiosRequestConfig
  ): Promise<Result<AxiosResponse, FetchError>> => {
    let err: FetchError = { message: "Unknow error" };

    try {
      let response: AxiosResponse;

      switch (config.method) {
        case "GET":
          response = await axios.get(config.url || "", config);
          return new Ok(response);
        case "POST":
          response = await axios.post(config.url || "", config.data, config);
          return new Ok(response);
        case "DELETE":
          response = await axios.delete(config.url || "", config);
          return new Ok(response);
        default:
          return new Err({ message: "Unsupported HTTP method" });
      }
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          switch (error.response.status) {
            case 400:
              console.error("Bad request:", error.response);
              err = {
                err_status: error.response.status,
                message: error.response.data,
              };
              return new Err(err);
            case 401:
              console.error("Unathorized: ", error.response);
              err = {
                err_status: error.response.status,
                message: "Unathorized. Please, authorize and try again",
              };
              return new Err(err);
            case 403:
              console.error("Forbidden:", error.response);
              err = {
                err_status: error.response.status,
                message:
                  "Forbidden. You don't have rights to make this request",
              };
              return new Err(err);
            case 404:
              console.error(
                "Not found, check the request: ",
                error.config?.url,
                error.response
              );
              err = {
                err_status: error.response.status,
                message: "Not found, check the request",
              };
              return new Err(err);
            case 500:
              console.error("Server error:", error.response);
              err = {
                err_status: error.response.status,
                message: "Internal server error. Please, try again later",
                recursive: true,
              };
              return new Err(err);
            default:
              console.error("API error: ", error.response.status, error);
              err = {
                err_status: error.response.status,
                message: error.response.statusText,
              };
              return new Err(err);
          }
        } else if (error.request) {
          err = {
            message: "Server isn't responding. Please, try again",
            recursive: true,
          };
          return new Err(err);
        } else {
          console.error("API Error", error);
          err = {
            message: `API error: ${error.message}`,
          };
          return new Err(err);
        }
      } else {
        console.error("Non-Axios:", error);
        err = {
          message: "Unknown Non-Axios error",
        };
        return new Err(err);
      }
    }
  };

  return {
    fetch_data,
  };
};

export default useAxios;
