import useBaseUrl from "@docusaurus/useBaseUrl";
import React from "react";
import clsx from "clsx";
import styles from "./HomepageFeatures.module.css";

type FeatureItem = {
	title: string;
	image: string;
	description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
	{
		title: "style",
		image: "/characters/character_1_Erik.svg",
		description: (
			<>
				Game with pixel art style, highly inspired by the the flying
				spaghetti monster, vikings and pirates.
			</>
		),
	},
	{
		title: "Online",
		image: "/img/undraw_docusaurus_tree.svg",
		description: (
			<>
				Game to play with all your friends. We aim in network
				performance and let you can communicate with all your friends.
			</>
		),
	},
	{
		title: "Open Source",
		image: "/img/undraw_docusaurus_react.svg",
		description: (
			<>
				Feel free to help me develop this project! You can find a lot of
				documentation here and if this is not enough, you can find more
				on my github repo!{" "}
			</>
		),
	},
];

function Feature({ title, image, description }: FeatureItem) {
	return (
		<div className={clsx("col col--4")}>
			<div className="text--center">
				<img
					className={styles.featureSvg}
					alt={title}
					src={useBaseUrl(image)}
				/>
			</div>
			<div className="text--center padding-horiz--md">
				<h3>{title}</h3>
				<p>{description}</p>
			</div>
		</div>
	);
}

export default function HomepageFeatures(): JSX.Element {
	return (
		<section className={styles.features}>
			<div className="container">
				<div className="row">
					{FeatureList.map((props, idx) => (
						<Feature key={idx} {...props} />
					))}
				</div>
			</div>
		</section>
	);
}
