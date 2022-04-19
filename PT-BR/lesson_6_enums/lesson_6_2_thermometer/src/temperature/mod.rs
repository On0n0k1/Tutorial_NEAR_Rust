pub mod temp_format;


use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


use temp_format::TempFormat;
use crate::utils::log;

/// Representa temperatura.
/// 
/// Possui um valor f32 para valor.
/// 
/// Possui um TempFormat para formato (Kelvin, Celsius ou Fahrenheit).
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Temperature {
    value: f32,
    temp_format: TempFormat,
}


impl Temperature {
    /// Constrói uma instância de temperatura. 
    /// 
    /// panic se o valor for menor do que zero absoluto.
    /// 
    fn new_assert_temp(value: f32, temp_format: TempFormat) -> Self {
        let (minimum, name) = match &temp_format {
            TempFormat::Celsius(t_format) => {(-273.15, t_format)},
            TempFormat::Fahrenheit(t_format) => {(-459.67, t_format)},
            TempFormat::Kelvin(t_format) => {(0., t_format)},
        };
        assert!(value >= minimum, "For temperature type {}, temperature value can not be lower than {}. Current: {}.", name, minimum, value);
        
        Temperature { 
            value, 
            temp_format,
        }
    }

    /// Cria uma instância de temperatura. O formato será definido pelo formato de sistema "temp_format".
    /// 
    /// "arg_temp" é o formato de temperatura da mensagem recebida.
    /// 
    /// Se arg_temp for diferente de temp_format. Seguirá estes passos:
    ///  - Cria uma instância de temperatura no formato de arg_temp;
    ///  - Converte seu formato para temp_format;
    ///  - Retorna a temperatura;
    /// 
    pub fn new(value: f32, temp_format: &TempFormat, arg_temp: Option<String>) -> Self {
        match arg_temp {
            None => {
                let temp_format = temp_format.clone();

                Temperature::new_assert_temp(value, temp_format)
            },
            Some(arg_string) => {
                let arg_format = TempFormat::new(&arg_string);

                let mut arg_temp = Temperature::new_assert_temp(value, arg_format);

                // Converte a temperatura da mensagem para a temperatura do sistema.
                arg_temp.convert(temp_format);
                arg_temp
            }
        }

    }

    /// Atualiza temperatura se for diferente. 
    /// 
    /// Retorna true se houver mudança.
    /// 
    pub fn update_temp_format(&mut self, temp_format: &TempFormat) -> bool {
        let comparison = self.temp_format == *temp_format;

        if !comparison {
            self.convert(temp_format);
        }

        !comparison
    }
    
    /// Converte o formato de temperatura para temp_format.
    /// 
    pub fn convert(&mut self, temp_format: &TempFormat){
        let current: TempFormat = self.temp_format.clone();

        let value = self.value;
        log(&format!("Converting temperature to system format. System: {}, Current: {}.", temp_format, &current));

        match (current, temp_format) {
            (TempFormat::Kelvin(_), &TempFormat::Celsius(_)) => {
                // C = K − 273.15
                self.value = value - 273.15;    
                self.temp_format = TempFormat::new("c");
            },
            (TempFormat::Kelvin(_), &TempFormat::Fahrenheit(_)) => {
                // F = (K – 273.15) × 9⁄5 + 32
                self.value = (value - 273.15) * 9.0 / 5.0 + 32.0;
                self.temp_format = TempFormat::new("f");
            },
            (TempFormat::Celsius(_), &TempFormat::Kelvin(_)) => {
                // K = C + 273.15
                self.value = value + 273.15;
                self.temp_format = TempFormat::new("k");
            },
            (TempFormat::Celsius(_), &TempFormat::Fahrenheit(_)) => {
                // F = C(9⁄5) + 32
                self.value = value * (9.0 / 5.0) + 32.0;
                self.temp_format = TempFormat::new("f");
            },
            (TempFormat::Fahrenheit(_), &TempFormat::Kelvin(_)) => {
                // K = (F − 32) × 5⁄9 + 273.15
                self.value = (value - 32.0) * 5.0 / 9.0 + 273.15;
                self.temp_format = TempFormat::new("k");
            },
            (TempFormat::Fahrenheit(_), &TempFormat::Celsius(_)) => {
                // C = (F − 32) × 5⁄9
                self.value = (value - 32.0) * 5.0 / 9.0;
                self.temp_format = TempFormat::new("c");
            },
            (_, _) => {
                // Todas alternativas diferentes foram consideradas. Isso considera todas as situações em que os tipos são iguais.
                // Portanto, não fazemos nada.
                return;
            }
        }
    }
}
