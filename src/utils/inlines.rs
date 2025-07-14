use orzklv::telegram::keyboard::Keyboard;
use teloxide::types::*;

use super::resources::prelude::Job;

pub static NO_INPUT: &str = r#"
<b>Salom foydalanuvchi!</b>

Siz hozir inlayn rejim ishga tushurdingiz. Bu xususiyat yordamida siz Uzinfocom tomonidan e'lon qilingan vakansiyalar qidirishingiz mumkin. Qidirishni boshlash uchun, quyida keltirilgan misol tariqasida yozing:

<code>@@uzinfojobful_bot &lt;biron so'z&gt;</code>
"#;

pub static NOT_FOUND: &str = r#"
<b>Ushbu so'rovga oid natija mavjud emas!</b>
Iltimos, boshqa nom bilan yoki keyinroq yana bir bor urinib ko'ring!
"#;

pub fn preview_generate(d: &Job) -> String {
    format!(
        "{} | {} | {} | {}",
        d.employ_type, d.experience, d.location, d.specialization.name
    )
}

pub fn view_generate(d: &Job) -> String {
    let mut result = String::new();

    result.push_str(&format!("<b>Ish:</b> {}\n", d.title));
    result.push_str(&format!("<b>Bandlik:</b> {}\n", d.employ_type));
    result.push_str(&format!("<b>Ofis:</b> {}\n", d.location));
    result.push_str(&format!("<b>Yo'nalish:</b> {}\n", d.specialization.name));
    result.push_str(&format!(
        "<b>Kerakli Tajriba:</b> <code>{} yil</code>\n",
        d.experience
    ));

    result
}

pub fn kb_generate(d: &Job) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    keyboard
        .url(
            "Ko'proq",
            &format!("https://uzinfocom.uz/company/career/{}", d.slug),
        )
        .unwrap()
}

pub fn err_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.switch_inline_current("Qayta urinib ko'ramizmi?", "Data")
}
