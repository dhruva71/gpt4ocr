Extract the contents of this image and present in the following format:
```json
{
  "name": <name>,
  "email": <email>,
  "phone": <phone>,
  "social_media": [
    {
      "platform": <platform>,
      "link": <link>
    },
    {
      "platform": <platform>,
      "link": <link>
    }
  ]
}
```

If any fields are missing, return null for that field. For example, if the email is missing, return null for the email field. If the social media field is missing, return an empty array for the social_media field. If the platform or link is missing in the social media field, return null for those fields.