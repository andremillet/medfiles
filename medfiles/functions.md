# Funções do Programa

## main()
Ponto de entrada do programa. Processa argumentos da linha de comando e executa comandos ou saudações.

## greet()
Saúda o usuário e apresenta um menu para escolher entre ver atendimentos ou prescrições.

## is_new_user()
Verifica se o arquivo user_info.json existe, indicando se o usuário já foi configurado.

## generate_token()
Gera um token aleatório de 6 dígitos para verificação.

## send_email(to, token)
Simula o envio de um email com o token de verificação (placeholder para implementação real).

## verify_token(expected_token)
Solicita a entrada do token do usuário com um temporizador de 10 minutos.

## create_user()
Coleta informações do usuário (nome, CPF, telefone, email) e verifica via email.

## is_prescription(line)
Verifica se uma linha começa com comandos de prescrição (!PRESCREVO, etc.).

## prescription_grabber(conduta_lines)
Filtra linhas que são prescrições de uma lista de linhas da conduta.

## conduta_handler()
Processa arquivos .med no diretório, extrai prescrições da seção [CONDUTA] e retorna uma lista.