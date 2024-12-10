use crate::core::{definitions::Milk, services::TryAcquire};

/// Wanting to spread the joy of sweet treats beyond the holidays, Santa let
/// build a wondrous factory to mass produce the perfect treat duo: cookies and
/// milk packaged as a 2-in-1 deal!
///
/// Branded as "Santa's Cookies and Milk," the product quickly became a
/// worldwide hit, delighting taste buds and hearts alike. The success was so
/// enormous that Santa now considers building more factories to meet the
/// growing demand! Santa has a bucket in the Cookies and Milk factory that can
/// store 5 liters of milk. As long as the bucket is not full, the bucket is
/// magically refilled with 1 liter of milk every second. Every time the
/// production line makes a pack of milk, 1 liter is taken from the bucket.
///
/// Withdrawing milk from the bucket when it is empty creates an empty pack of
/// milk, which can not be accepted in this factory. Santa needs a way to ensure
/// this never ever happens!
///
/// Make a POST endpoint /9/milk that makes sure the bucket is never empty. It
/// should take no request body as input. If there is milk in the bucket,
/// respond with 200 OK and the string Milk withdrawn\n. If there is no milk in
/// the bucket, respond with 429 Too Many Requests and the string No milk
/// available\n.
///
/// See [challenge page](https://console.shuttle.dev/shuttlings/cch24/challenge/9) for details.
pub struct GetMilkOperation<'a, T> {
    pub rate_limit_service: &'a T,
}

impl<'a, T> GetMilkOperation<'a, T>
where
    T: TryAcquire,
{
    pub fn execute(&self) -> Option<Milk> {
        if self.rate_limit_service.try_acquire() {
            return Some(Milk::new());
        }

        None
    }
}
