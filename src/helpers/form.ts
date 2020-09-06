const encode = (data: Map<string, string>) => {
  return Object.keys(data)
    .map(
      (key) =>
        encodeURIComponent(key) +
        "=" +
        encodeURIComponent(data.get(key) as string)
    )
    .join("&");
};

export const submit = (
  e: React.FormEvent<HTMLFormElement>,
  set_status: (value: string) => void
) => {
  let target: any = e.target as any;

  fetch("/", {
    method: "POST",
    headers: { "Content-Type": "application/x-www-form-urlencoded" },
    body: encode(
      new Map([
        ["form-name", "contact"],
        ["email", target.email.value],
        ["name", target.name.value],
        ["message", target.message.value],
      ])
    ),
  })
    .then(() => set_status("Success!"))
    .catch((error) => set_status(error));

  e.preventDefault();
};
