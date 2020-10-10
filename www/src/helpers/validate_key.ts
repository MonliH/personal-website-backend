const validate_key = async (key: string): Promise<boolean> => {
  const request_options = {
    method: "POST",
    body: key,
  };

  const is_correct = await fetch("/api/admin/key", request_options);

  return is_correct.ok;
};

export default validate_key;
