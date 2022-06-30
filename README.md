# tartaros-telegram

A database to blacklist to ban spammers and scammers on Telegram right into the depths of Tartaros, because that's where
scums belong.

## Usage

You can access the following open endpoints.

| HTTP-Request | Endpoint | Expected Result                                                                                                |
|--------------|---------|----------------------------------------------------------------------------------------------------------------|
| `GET`        | `/`      | All blacklisted users.                                                                                         |
| `GET`        | `/<id>` | A blacklisted user by its Telegram user_id or 404 if none was found.                                           |

These endpoints require the correct `token` in the header of the request.

| HTTP-Request | Endpoint | Expected Result                                                                                                |
|--------------|----------|----------------------------------------------------------------------------------------------------------------|
| `POST        | `/`      | Add a user to be blacklisted in json format with attributes `id` and the `message` this user was reported for. |
| `DELETE`     | `/<id>`  | Remove a user from the blacklist by his `user_id` on Telegram.                                                 |