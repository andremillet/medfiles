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

## generate_html_header(title)
Gera o cabeçalho HTML com título, CSS completo e abertura da tag body.

## generate_recipe_section(recipe_list_html)
Gera a seção HTML para exibir a lista de receitas em formato ordenado.

## generate_changes_section(returns_content)
Gera a seção HTML para exibir as alterações das prescrições com formatação colorida.

## generate_timeline_section(graph_html)
Gera a seção HTML para exibir a timeline de evolução das prescrições.

## generate_html_footer()
Gera o fechamento das tags HTML (body e html).

## generate_complete_html(recipe_list_html, returns_content, graph_html)
Função principal que combina todas as seções HTML em um documento completo.

## parse_prescription_to_list(recipe_content)
Converte o conteúdo da receita em uma lista HTML ordenada com formatação adequada.

## prescription_graphs_html(changes)
Converte os dados de alterações em uma timeline HTML visual com marcadores coloridos.

## ansi_to_html(text)
Converte códigos ANSI de cores (usados em diffs) para tags HTML span com classes CSS.

## extract_medication_from_return(return_msg)
Extrai o nome da medicação das mensagens de retorno para filtragem de alterações.