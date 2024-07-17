export type TUser = {
  id?: { tb: "account"; id: { String: string } };
  name: string;
  init_data: string;
  balance?: number;
  passes?: number;
  status: boolean;
};
