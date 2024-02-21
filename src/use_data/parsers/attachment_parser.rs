use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::Attachment;
use crate::use_data::utils::word_fits_filters;
use rand::Rng;

pub fn parse_attachments(
    attachments: Vec<Attachment>,
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
) -> Vec<Attachment> {
    let mut attachment_list: Vec<Attachment> = Vec::new();

    if let Some(amount) = amount {
        if random {
            let mut rng = rand::thread_rng();
            while attachment_list.len() as i32 != amount {
                let random_index = rng.gen_range(0..attachments.len());
                let attachment_at_index = attachments[random_index].clone();
                if !word_fits_filters(
                    &attachment_at_index.orth,
                    &attachment_at_index.pos,
                    &pos_list,
                    &max,
                    &min,
                    &exact,
                ) {
                    continue;
                }
                attachment_list.push(attachment_at_index);
            }
        } else {
            for attachment in attachments {
                if !word_fits_filters(
                    &attachment.orth,
                    &attachment.pos,
                    &pos_list,
                    &max,
                    &min,
                    &exact,
                ) {
                    continue;
                }

                attachment_list.push(attachment);
                if attachment_list.len() as i32 == amount {
                    break;
                }
            }
        }
    } else {
        for attachment in attachments {
            if !word_fits_filters(
                &attachment.orth,
                &attachment.pos,
                &pos_list,
                &max,
                &min,
                &exact,
            ) {
                continue;
            }

            attachment_list.push(attachment);
        }
    }

    attachment_list
}
