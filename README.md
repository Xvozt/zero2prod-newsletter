# Email newsletter
With this project I will try to implement Email newsletter from zero, following the book "Zero To Production In Rust".

## User stories to implement

We will fulfill three user stories:

- [ ] As a blog **visitor**,
I want to subscribe to the newsletter,
So that I can receive email updates when new content is published on the blog;
- [ ] As the blog **author**,
I want to send an email to all my subscribers,
So that I can notify them when new content is published;
- [ ] As a **subscriber**,
I want to be able to unsubscribe from the newsletter,
So that I can stop receiving email updates from the blog.

We will not add features to

- manage multiple newsletters;
- segment subscribers in multiple audiences;
- track opening and click rates.


## Technical stuff

### API

```
/health_check GET

/subscriptions POST

/subscriptions/confirm GET
```

## Confirmation link for new subscriber plan

Every time a user wants to subscribe to our newsletter they fire a POST /subscriptions request. Our request handler will:

- [x] add their details to our database in the subscriptions table, with status equal to pending_confirmation;
- [x] generate a (unique) subscription_token;
- [x] store subscription_token in our database against their id in a subscription_tokens table;
- [x] send an email to the new subscriber containing a link structured as https://<our-api-domain>/subscriptions/confirm?token=<subscription_token>;
- [x] return a 200 OK.

Once they click on the link, a browser tab will open up and a GET request will be fired to our GET /subscriptions/confirm endpoint. Our request handler will:

- [x] retrieve subscription_token from the query parameters;
- [x] retrieve the subscriber id associated with subscription_token from the subscription_tokens table;
- [x] update the subscriber status from pending_confirmation to active in the subscriptions table;
- [w] return a 200 OK.


## Quality of life stuff:
- [ ] If user tries to subscribe twice I need to make sure that he receives two confirmation emails;
- [ ] What happens if a user clicks on a confirmation link twice?
- [ ] What happens if the subscription token is well-formatted but non-existent?
- [ ] Add validation on the incoming token, I am currently passing the raw user input straight into a query;
- [ ] Use a proper templating solution for emails (e.g. [tera](https://crates.io/crates/tera));
Once all is done i need to:
- [ ] add proper integration with postmark in production