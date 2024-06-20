# PARSER

Afin d'aider à la lecture du code, voici quelques explications sur le Parser.

## Grammaire

La grammaire était la première étape de la création du parser.

Une description détaillée peut être trouvée sur
le [site internet](https://dibi-programming-language.github.io/SkribiDocFr/Grammaire.html).

## Principe de base

Le "parser" a pour but de transformer une liste de tokens en un arbre décrivant la syntaxe du code.

Chaque nœud de l'arbre représente une règle de grammaire. Certains nœuds sont uniquement là pour les priorités,
tandis que d'autres devrons exécuter des actions.

Tous les types de nœuds suivent une logique précise.

### Logique des nœuds

- Chaque nœud contient des enfants, rangés d'une manière spécifique à chaque nœud.
- Tout nœud est accompagné de la mention `#[derive(PartialEq)]` pour permettre la comparaison de deux arbres.
- Le display est implémenté à l'aide d'une macro `impl_debug!(ClassDec);`. Pour utiliser cette macro, il est important
  de penser à réaliser les imports.
- Afin d'utiliser la macro, il est important d'implémenter `GraphDisplay` pour chaque nœud. Ce trait permet de générer
  un string représentant l'entièrement de l'arbre au format Mermaid.
- Pour le parsing, cela a été modifié pendant le développement. Pour les premières classes, une fonction est utilisée,
  tandis que pour les plus récentes, c'est une méthode `parse` et une méthode `new` qui sont utilisées.
- Les erreurs sont toutes d'un type personnalisé : la plupart des méthodes de parsing renvoient
  un `Option<Result<[type de la node], CustomError>>`, avec `option`ou `result` en moins en fonction des cas qui ne
  peuvent pas apparaitre.

### Tester les nœuds

Tous les tests sont situés dans le dossier `src/tests/parse_tests/`.
