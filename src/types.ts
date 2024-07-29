export type TUser = {
  id?: { tb: "account"; id: { String: string } };
  name: string;
  account_type: "Blum" | "Dogs";
  init_data: string;
  balance?: number;
  passes?: number;
  status: boolean;
};
