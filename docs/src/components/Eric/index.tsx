import React from "react";
import useBaseUrl from "@docusaurus/useBaseUrl";

const Eric = ({
	width,
	height,
	svg_path,
	alt,
}: {
	width: string;
	height: string;
	svg_path: string;
	alt?: string;
}) => {
	return (
		<div style={{ height: height, width: width }}>
			<img src={useBaseUrl(svg_path)} alt={alt ? alt : "img_"} />
		</div>
	);
};

export default Eric;
