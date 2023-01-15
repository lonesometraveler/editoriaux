# editoriaux

This is a Rust application that display the title and the url of the 5 latest editorial articles from [the New York Times](https://www.nytimes.com) and [Le Monde](https://www.lemonde.fr).

## Usage

Clone the repository and build the application by running:


```
git clone https://github.com/lonesometraveler/editoriaux.git
cd editoriaux
cargo run --release
```

This will create a release binary in the target/release directory and run the application.


If everything goes well, you should see the 5 latest editorial articles from the New York Times and Le Monde printed to the console.


## Output Example

Here is an example of the output you can expect from the application:

```
Le Monde
* Au Pérou, stopper la spirale de la violence / "https://www.lemonde.fr/idees/article/2023/01/14/au-perou-stopper-la-spirale-de-la-violence_6157871_3232.html"
* Etats-Unis: les républicains otages du jusqu’au-boutisme / "https://www.lemonde.fr/idees/article/2023/01/13/etats-unis-les-republicains-otages-du-jusqu-au-boutisme_6157728_3232.html"
* Retraites: l’irréductible fracture / "https://www.lemonde.fr/idees/article/2023/01/11/retraites-l-irreductible-fracture_6157404_3232.html"
* Soutenir l’Ukrainepour assurer la paix / "https://www.lemonde.fr/idees/article/2023/01/10/soutenir-l-ukraine-pour-assurer-la-paix_6157279_3232.html"
* Menaces sur la démocratie brésilienne / "https://www.lemonde.fr/idees/article/2023/01/09/menaces-sur-la-democratie-bresilienne_6157127_3232.html"

NY Times
* A Deal to Help South Africa Is a Breakthrough for the World / "https://www.nytimes.com//2023/01/14/opinion/climate-change-south-africa.html"
* A Promising New Path to Protect Abortion Access / "https://www.nytimes.com//2023/01/07/opinion/abortion-rights.html"
* A Bricklayer Rebuilds His Life / "https://www.nytimes.com//2022/12/26/opinion/neediest-cases-foster-housing.html"
* How Americans Can Stand Against Extremism / "https://www.nytimes.com//2022/12/24/opinion/anti-trans-violence.html"
* The Last Lesson of the Jan. 6 Committee / "https://www.nytimes.com//2022/12/22/opinion/editorials/jan-6-committee-report.html"
```