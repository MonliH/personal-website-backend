const encode = (data: Array<[string, string]>) => {
  return data.map(
      ([key, value]) =>
        encodeURIComponent(key) +
        "=" +
        encodeURIComponent(value)
    )
    .join("&");
};

export const submit = (
  e: React.FormEvent<HTMLFormElement>,
  set_status: (value: string) => void
) => {
  let target: any = e.target as any;
  let body = encode(
      [
        ["form-name", "contact"],
        ["email", target.email.value],
        ["name", target.name.value],
        ["message", target.message.value],
      ]
    );

  console.log(body);

  fetch("/", {
    method: "POST",
    headers: { "Content-Type": "application/x-www-form-urlencoded" },
    body,
  })
    .then(() => set_status("Success!"))
    .catch((error) => set_status(error));

  e.preventDefault();
};
